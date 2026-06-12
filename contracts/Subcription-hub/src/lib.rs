#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Env, String, Address, symbol_short, Symbol};

// --- CẤU TRÚC DỮ LIỆU (DATA STRUCTURES) ---

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Plan {
    pub name: String,
    pub price: u64,       // Giá trị mang tính chất ghi nhận trạng thái (Payment external)
    pub duration: u64,    // Tính bằng giây (Seconds) để tiện test nhanh
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Subscription {
    pub plan_name: String,
    pub user: Address,
    pub start_time: u64,
    pub end_time: u64,
    pub auto_renew: bool,
    pub is_active: bool,
}

// --- KHÓA LƯU TRỮ TRÊN BLOCKCHAIN (STORAGE KEYS) ---
#[contracttype]
pub enum DataKey {
    Admin,
    Plan(String),
    Subscription(Address),
}

#[contract]
pub struct ChainSubscriptionHub;

#[contractimpl]
impl ChainSubscriptionHub {

    /// 1. Set Admin: Kích hoạt contract và gán quyền Admin cho người deploy
    pub fn initialize(env: Env, admin: Address) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("Contract đã được khởi tạo trước đó!");
        }
        env.storage().instance().set(&DataKey::Admin, &admin);
    }

    /// Kiểm tra xem người gọi hàm có phải Admin không
    fn check_admin(env: &Env) -> Address {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        admin.require_auth();
        admin
    }

    /// 2. Create Plans: Admin tạo hoặc cập nhật các gói đăng ký
    /// Bạn có thể tạo "Basic" (60s), "Standard" (300s), "Premium" (600s) tùy ý khi gọi hàm
    pub fn create_plan(env: Env, name: String, price: u64, duration_seconds: u64) {
        Self::check_admin(&env);
        
        let plan = Plan {
            name: name.clone(),
            price,
            duration: duration_seconds,
        };
        
        // Lưu plan vào bộ nhớ của Contract
        env.storage().instance().set(&DataKey::Plan(name), &plan);
    }

    /// 3. Subscribe: Người dùng đăng ký một gói thành viên
    pub fn subscribe(env: Env, user: Address, plan_name: String, auto_renew: bool) {
        user.require_auth();

        // Kiểm tra xem Plan này có tồn tại không
        let plan_key = DataKey::Plan(plan_name.clone());
        if !env.storage().instance().has(&plan_key) {
            panic!("Gói đăng ký không tồn tại!");
        }
        let plan: Plan = env.storage().instance().get(&plan_key).unwrap();

        // Lấy thời gian hiện tại của Block (tính bằng giây)
        let current_time = env.ledger().timestamp();
        let end_time = current_time + plan.duration;

        let sub = Subscription {
            plan_name,
            user: user.clone(),
            start_time: current_time,
            end_time,
            auto_renew,
            is_active: true,
        };

        // Lưu thông tin đăng ký của User
        env.storage().instance().set(&DataKey::Subscription(user), &sub);
    }

    /// 4. Auto Renew: Gia hạn khi gói cũ đã hết hạn và user có bật auto_renew
    /// Bất kỳ ai cũng có thể kích hoạt hộ (hoặc hệ thống bot gọi) bằng cách truyền Address của user vào
    pub fn auto_renew(env: Env, user: Address) {
        let sub_key = DataKey::Subscription(user.clone());
        if !env.storage().instance().has(&sub_key) {
            panic!("Người dùng chưa đăng ký gói nào!");
        }

        let mut sub: Subscription = env.storage().instance().get(&sub_key).unwrap();
        let current_time = env.ledger().timestamp();

        // Điều kiện gia hạn: Phải bật auto_renew, đang active, và thời gian hiện tại đã vượt quá thời gian hết hạn
        if sub.auto_renew && sub.is_active && current_time >= sub.end_time {
            let plan_key = DataKey::Plan(sub.plan_name.clone());
            let plan: Plan = env.storage().instance().get(&plan_key).unwrap();

            // Cập nhật chu kỳ mới dựa trên thời điểm hết hạn cũ (hoặc thời điểm hiện tại nếu bị trễ)
            let base_time = if current_time > sub.end_time { current_time } else { sub.end_time };
            sub.start_time = base_time;
            sub.end_time = base_time + plan.duration;

            // Lưu lại trạng thái mới cập nhật
            env.storage().instance().set(&sub_key, &sub);
        } else {
            panic!("Không đủ điều kiện tự động gia hạn (Chưa hết hạn hoặc User tắt Auto-Renew)");
        }
    }

    /// 5. Cancel: Người dùng hủy gói (Tắt tính năng gia hạn hoặc dừng ngay lập tức)
    pub fn cancel_subscription(env: Env, user: Address) {
        user.require_auth();

        let sub_key = DataKey::Subscription(user.clone());
        if !env.storage().instance().has(&sub_key) {
            panic!("Không tìm thấy thông tin đăng ký để hủy!");
        }

        let mut sub: Subscription = env.storage().instance().get(&sub_key).unwrap();
        
        // Hủy đăng ký: Tắt active và tắt luôn tự động gia hạn tương lai
        sub.is_active = false;
        sub.auto_renew = false;

        env.storage().instance().set(&sub_key, &sub);
    }

    /// 6. Query: Kiểm tra thông tin gói đăng ký của một User bất kỳ (Public ai cũng xem được)
    pub fn get_subscription(env: Env, user: Address) -> Option<Subscription> {
        let sub_key = DataKey::Subscription(user);
        if env.storage().instance().has(&sub_key) {
            Some(env.storage().instance().get(&sub_key).unwrap())
        } else {
            None
        }
    }

    /// Xem thông tin chi tiết cấu hình của một Plan
    pub fn get_plan(env: Env, name: String) -> Option<Plan> {
        let plan_key = DataKey::Plan(name);
        if env.storage().instance().has(&plan_key) {
            Some(env.storage().instance().get(&plan_key).unwrap())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test;