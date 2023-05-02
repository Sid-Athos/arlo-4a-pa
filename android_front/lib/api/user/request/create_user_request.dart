
class CreateUserRequest {
  String nickname;
  String email;
  String password;

  CreateUserRequest({
    required this.nickname,
    required this.email,
    required this.password,
  });

  Map<String, dynamic> toJson() => {
    "nickname": nickname,
    "email": email,
    "password": password,
  };
}
