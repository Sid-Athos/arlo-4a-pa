import 'package:flutter/widgets.dart';
import 'package:flutter_webrtc/flutter_webrtc.dart';
import 'package:miku/provider/user_ice_candidate_provided.dart';
import 'package:miku/provider/user_sdp_provided.dart';
import 'package:web_socket_channel/web_socket_channel.dart';

import '../api/game_manager/api_game_manager.dart';
import '../api/game_manager/response/message_response_ws.dart';
import '../model/ice_candidate_model.dart';
import '../model/rtc_session.dart';
import '../model/user_model.dart';

class GameProvider extends ChangeNotifier {

  List<MessageResponseWS> messages;
  List<UserSdp> offerSDP;
  List<UserSdp> answerSDP;
  List<UserIceCandidate> iceCandidates;
  bool isShowChat = false;
  bool isShowCall = false;
  WebSocketChannel? channel;
  List<RtcSession> rtcSessions = [];
  MediaStream? localStream;
  RTCVideoRenderer localRenderer = RTCVideoRenderer();

  GameProvider({
    required this.messages,
    required this.offerSDP,
    required this.answerSDP,
    required this.iceCandidates,
    required this.isShowChat,
    required this.channel,
  });

  initLocalStream() async {
    await localRenderer.initialize();

    localStream = await navigator.mediaDevices.getUserMedia({
      'audio': true,
      'video': true,
    });

    localRenderer.srcObject = localStream;
    notifyListeners();
  }

  void answerSdpOffer(String sdp, int userId) async {
    RtcSession rtcSession = RtcSession(userId);

    await rtcSession.initPeerConnection(localStream!, channel!);

    await rtcSession.answerSdp(sdp, channel!);

    rtcSessions.add(rtcSession);
    notifyListeners();
  }

  void setRemoteAnswer(String sdp, int userId) async {
    for (RtcSession rtcSession in rtcSessions) {
      if (rtcSession.userId == userId) {
        rtcSession.setRemoteAnswer(sdp);
        notifyListeners();
      }
    }
  }

  void addIceCandidate(ICECandidate iceCandidate, int userId) async {
    for (RtcSession rtcSession in rtcSessions) {
      if (rtcSession.userId == userId) {
        rtcSession.addIceCandidate(iceCandidate);
        notifyListeners();
      }
    }
  }

  void joinCall() async {
    await initLocalStream();

    List<User> usersInCall = await ApiGameManager.joinRtcSession();

    for (User userInCall in usersInCall) {
      RtcSession rtcSession = RtcSession(userInCall.id);

      await rtcSession.initPeerConnection(localStream!, channel!);
      await rtcSession.sendOffer(channel!);

      rtcSessions.add(rtcSession);
      notifyListeners();
    }
  }

  void addMessage(MessageResponseWS message) {
    messages.add(message);
    notifyListeners();
  }

  void toggleChat(bool isShowChat) {
    this.isShowChat = isShowChat;
    notifyListeners();
  }

  void toggleCall(bool isShowCall) {
    this.isShowCall = isShowCall;
    notifyListeners();
  }
}