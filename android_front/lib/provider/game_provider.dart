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
  bool isShowChat = false;
  bool isShowCall = false;
  bool inCall = false;
  WebSocketChannel? channel;
  List<RtcSession> rtcSessions = [];
  MediaStream? localStream;
  RTCVideoRenderer localRenderer = RTCVideoRenderer();

  GameProvider({
    required this.messages,
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

  void answerSdpOffer(String sdp, User user) async {
    RtcSession rtcSession = RtcSession(user);

    await rtcSession.initPeerConnection(localStream!, channel!);

    await rtcSession.answerSdp(sdp, channel!);

    rtcSessions.add(rtcSession);
    notifyListeners();
  }

  void setRemoteAnswer(String sdp, User user) async {
    for (RtcSession rtcSession in rtcSessions) {
      if (rtcSession.user.id == user.id) {
        rtcSession.setRemoteAnswer(sdp);
        notifyListeners();
      }
    }
  }

  void addIceCandidate(ICECandidate iceCandidate, User user) async {
    for (RtcSession rtcSession in rtcSessions) {
      if (rtcSession.user.id == user.id) {
        rtcSession.addIceCandidate(iceCandidate);
        notifyListeners();
      }
    }
  }

  void joinCall() async {
    await initLocalStream();

    List<User> usersInCall = await ApiGameManager.joinRtcSession();

    for (User userInCall in usersInCall) {
      RtcSession rtcSession = RtcSession(userInCall);

      await rtcSession.initPeerConnection(localStream!, channel!);
      await rtcSession.sendOffer(channel!);

      rtcSessions.add(rtcSession);
    }

    inCall = true;
    notifyListeners();
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

  void toggleLocalAudio(bool enabled) {
    localStream!.getAudioTracks()[0].enabled = enabled;
    notifyListeners();
  }

  void toggleLocalVideo(bool enabled) {
    localStream!.getVideoTracks()[0].enabled = enabled;
    notifyListeners();
  }

  void leaveCall() async {
    await ApiGameManager.leaveRtcSession();
    for (RtcSession rtcSession in rtcSessions) {
      rtcSession.peerConnection?.close();
    }
    rtcSessions = [];
    localStream?.dispose();
    inCall = false;
    notifyListeners();
  }

  void userLeftCall(User user) {
    for (int i = 0; i < rtcSessions.length; i++) {
      if (rtcSessions[i].user.id == user.id) {
        rtcSessions[i].peerConnection?.close();
        rtcSessions.remove(rtcSessions[i]);
        notifyListeners();
      }
    }
  }
}