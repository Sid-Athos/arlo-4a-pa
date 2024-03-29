import '../../../model/user/user_model.dart';

class UserResponseMapper {

  static User fromJson(Map<String, dynamic> json) {
    return User(
      id: json['id'],
      pseudo: json['pseudo'],
      email: json['email'],
      admin: json['admin'],
      experience: json['experience'],
      level: json['level'],
    );
  }
}