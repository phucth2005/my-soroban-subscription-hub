#![cfg(test)]
use super::*;
use soroban_sdk::{Env, String, Address};

#[test]
fn test_subscription_flow() {
    // 1. Khởi tạo môi trường giả lập Soroban
    let env = Env::default();
    env.mock_all_auths(); // Bật tính năng giả lập xác thực quyền (require_auth)

    // 2. Đăng ký Contract vào môi trường test
    let contract_id = env.register_contract(None, ChainSubscriptionHub);
    let client = ChainSubscriptionHubClient::new(&env, &contract_id);

    // 3. Tạo các địa chỉ ví giả lập cho Admin và User
    let admin = Address::generate(&env);
    let user = Address::generate(&env);

    // 4. Chạy hàm Khởi tạo Contract với quyền Admin
    client.initialize(&admin);

    // 5. Thử nghiệm tính năng tạo Plan "Basic" (Thời hạn 60 giây)
    let plan_name = String::from_str(&env, "Basic");
    client.create_plan(&plan_name, &10, &60);

    // Kiểm tra xem Plan đã lưu đúng thông tin cấu hình chưa
    let saved_plan = client.get_plan(&plan_name).unwrap();
    assert_eq!(saved_plan.price, 10);
    assert_eq!(saved_plan.duration, 60);

    // 6. Thử nghiệm tính năng Đăng ký (User đăng ký gói Basic, bật Auto Renew)
    client.subscribe(&user, &plan_name, &true);

    // Kiểm tra trạng thái gói sau khi đăng ký thành công
    let sub = client.get_subscription(&user).unwrap();
    assert_eq!(sub.is_active, true);
    assert_eq!(sub.auto_renew, true);

    // 7. GIẢ LẬP THỜI GIAN TRÔI QUA (Tua nhanh thời gian trên Blockchain)
    // Lấy thời gian hiện tại của block và cộng thêm 65 giây để gói Basic (60s) bị hết hạn
    let current_ledger_time = env.ledger().timestamp();
    env.ledger().set_timestamp(current_ledger_time + 65);

    // 8. Thử nghiệm tính năng Tự động gia hạn (Auto Renew) sau khi hết hạn
    client.auto_renew(&user);

    // Kiểm tra xem thời gian hết hạn mới (end_time) đã được đẩy lên chu kỳ tiếp theo chưa
    let renewed_sub = client.get_subscription(&user).unwrap();
    assert!(renewed_sub.end_time > sub.end_time); 
    
    // 9. Thử nghiệm tính năng Hủy gói (Cancel)
    client.cancel_subscription(&user);
    let cancelled_sub = client.get_subscription(&user).unwrap();
    assert_eq!(cancelled_sub.is_active, false);
    assert_eq!(cancelled_sub.auto_renew, false);
}