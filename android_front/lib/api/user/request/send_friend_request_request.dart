
class SendFriendRequestRequest {
  int recipientId;

  SendFriendRequestRequest({
    required this.recipientId,
  });

  Map<String, dynamic> toJson() => {
    "recipient_id": recipientId,
  };
}
