
class ChangePasswordRequest {
  String old_password;
  String new_password;

  ChangePasswordRequest({
    required this.old_password,
    required this.new_password,
  });

  Map<String, dynamic> toJson() => {
    "old_password": old_password,
    "new_password": new_password,
  };
}
