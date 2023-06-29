import 'package:flutter/widgets.dart';
import 'package:miku/provider/user_ice_candidate_provided.dart';
import 'package:miku/provider/user_sdp_provided.dart';

import '../api/game_manager/response/message_response_ws.dart';
import '../model/ice_candidate_model.dart';

class GameProvider extends ChangeNotifier {

  List<MessageResponseWS> messages;
  List<UserSdp> offerSDP;
  List<UserSdp> answerSDP;
  List<UserIceCandidate> iceCandidates;
  bool isShowChat = false;

  GameProvider({
    required this.messages,
    required this.offerSDP,
    required this.answerSDP,
    required this.iceCandidates,
    required this.isShowChat,
  });

  void addMessage(MessageResponseWS message) {
    messages.add(message);
    notifyListeners();
  }

  void addIceCandidate(ICECandidate iceCandidate, int userId) {
    iceCandidates.add(UserIceCandidate(userId: userId, iceCandidates: iceCandidate, computed: false));
    notifyListeners();
  }

  void addOffer(String sdp, int userId) {
    offerSDP.add(UserSdp(userId: userId, sdp: sdp, computed: false));
    notifyListeners();
  }

  void addAnswer(String sdp, int userId) {
    answerSDP.add(UserSdp(userId: userId, sdp: sdp, computed: false));
    notifyListeners();
  }

  void toggleChat(bool isShowChat) {
    this.isShowChat = isShowChat;
    notifyListeners();
  }
}