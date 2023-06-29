
import 'package:miku/model/ice_candidate_model.dart';

class UserIceCandidate {

  int userId;
  ICECandidate iceCandidates;
  bool computed;

  UserIceCandidate({
    required this.userId,
    required this.iceCandidates,
    required this.computed
  });
}