import 'package:flutter/cupertino.dart';

import '../api/game_manager/response/message_response_ws.dart';
import '../model/ice_candidate_model.dart';

class GameProvider extends ChangeNotifier {

  List<MessageResponseWS> messages;
  String sdp;
  List<ICECandidate> iceCandidates;

  GameProvider({
    required this.messages,
    required this.sdp,
    required this.iceCandidates
  });

  void addMessage(MessageResponseWS message) {
    messages.add(message);
    notifyListeners();
  }

  void addIceCandidate(ICECandidate iceCandidate) {
    iceCandidates.add(iceCandidate);
    notifyListeners();
  }

  void updateSDP(String sdp) {
    this.sdp = sdp;
    notifyListeners();
  }
}