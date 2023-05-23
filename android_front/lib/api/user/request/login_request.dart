
class LoginRequest {
  String nickname;
  String password;

  LoginRequest({
    required this.nickname,
    required this.password,
  });

  Map<String, dynamic> toJson() => {
    "nickname": nickname,
    "password": password,
  };
}
