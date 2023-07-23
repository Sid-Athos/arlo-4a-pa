import 'package:miku/model/friend/friend_list_model.dart';
import 'package:miku/mapper/user/user_response_mapper.dart';

class FriendListResponseMapper {

  static FriendList fromJson(Map<String, dynamic> json) {
    return FriendList(
      id: json['id'],
      applicantId: json['applicant_id'],
      recipientId: json['recipient_id'],
      applicant: UserResponseMapper.fromJson(json["applicant"]),
      recipient: UserResponseMapper.fromJson(json["recipient"]),
      accepted: json['accepted'],
    );
  }
}