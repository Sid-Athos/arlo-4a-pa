import 'dart:developer' as developer;

class MessageResponseWS {
  final String message;
  final int fromUser;

  MessageResponseWS({
    required this.message,
    required this.fromUser,
  });

  factory MessageResponseWS.fromJson(Map<String, dynamic> json) {
    return MessageResponseWS(
      message: json['message'],
      fromUser: json['from_user'],
    );
  }

  static void compute(Map<String, dynamic> json) {

    MessageResponseWS messageResponse = MessageResponseWS.fromJson(json['Message']);

    developer.log(messageResponse.message);
    developer.log(messageResponse.fromUser.toString());
  }
}