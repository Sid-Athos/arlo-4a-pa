
import 'package:miku/model/friend_list_model.dart';

class FriendListResponseMapper {

  static FriendList fromJson(Map<String, dynamic> json) {
    return FriendList(
      id: json['id'],
      applicantId: json['applicant_id'],
      recipientId: json['recipient_id'],
      accepted: json['accepted'],
    );
  }
}