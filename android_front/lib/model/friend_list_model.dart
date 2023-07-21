
import 'package:miku/model/user_model.dart';

class FriendList {
  int id;
  int applicantId;
  int recipientId;
  bool accepted;
  User applicant;
  User recipient;

  FriendList({
    required this.id,
    required this.applicantId,
    required this.recipientId,
    required this.accepted,
    required this.applicant,
    required this.recipient,
  });
}
