import 'dart:developer' as developer;

import 'package:miku/model/mapper/user_response_mapper.dart';
import 'package:miku/model/user_model.dart';

class EmoteResponseWS {
  final String emote;
  final User fromUser;

  EmoteResponseWS({
    required this.emote,
    required this.fromUser,
  });

  factory EmoteResponseWS.fromJson(Map<String, dynamic> json) {
    return EmoteResponseWS(
      emote: json['emote'],
      fromUser: UserResponseMapper.fromJson(json['from_user']),
    );
  }

  static void compute(Map<String, dynamic> json) {

    EmoteResponseWS emoteResponseWS = EmoteResponseWS.fromJson(json['Message']);

    developer.log(emoteResponseWS.emote);
    developer.log(emoteResponseWS.fromUser.toString());
  }
}