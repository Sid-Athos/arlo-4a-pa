import 'dart:developer' as developer;

class MessageResponse {
  final String message;
  final int fromUser;

  MessageResponse({
    required this.message,
    required this.fromUser,
  });

  factory MessageResponse.fromJson(Map<String, dynamic> json) {
    return MessageResponse(
      message: json['message'],
      fromUser: json['from_user'],
    );
  }

  static void compute(Map<String, dynamic> json) {

    MessageResponse messageResponse = MessageResponse.fromJson(json['Message']);

    developer.log(messageResponse.message);
    developer.log(messageResponse.fromUser.toString());
  }
}