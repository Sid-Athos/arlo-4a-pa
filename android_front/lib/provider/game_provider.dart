import 'package:flutter/widgets.dart';

import '../api/game_manager/response/message_response_ws.dart';
import '../model/ice_candidate_model.dart';

class GameProvider extends ChangeNotifier {

  List<MessageResponseWS> messages;
  String offerSDP;
  String answerSDP;
  List<ICECandidate> iceCandidates;
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

  void addIceCandidate(ICECandidate iceCandidate) {
    iceCandidates.add(iceCandidate);
    notifyListeners();
  }

  void addOffer(String sdp) {
    this.offerSDP = sdp;
    notifyListeners();
  }

  void addAnswer(String sdp) {
    this.answerSDP = sdp;
    notifyListeners();
  }

  void toggleChat(bool isShowChat) {
    this.isShowChat = isShowChat;
    notifyListeners();
  }
}