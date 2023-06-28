import 'dart:convert';

import 'package:flutter_webrtc/flutter_webrtc.dart';
import 'package:web_socket_channel/web_socket_channel.dart';

import 'ice_candidate_model.dart';

class RtcSession {
  static const Map<String, dynamic> configuration = {
    'iceServers': [
      {'url': 'stun:stun1.l.google.com:19302'},
      {'url': 'stun:stun2.l.google.com:19302'}
    ],
  };
  static const encoder = JsonEncoder();

  RTCPeerConnection? peerConnection;
  MediaStream? remoteStream;
  RTCVideoRenderer remoteRenderer = RTCVideoRenderer();
  List<ICECandidate> iceCandidates = [];
  bool isRemoteSet = false;
  int userId;

  RtcSession(this.userId) {
    remoteRenderer.initialize();
  }

  void initPeerConnection(
      MediaStream localStream, WebSocketChannel channel) async {
    peerConnection = await createPeerConnection(configuration);

    localStream.getTracks().forEach((track) {
      peerConnection?.addTrack(track, localStream);
    });

    peerConnection?.onIceCandidate = (RTCIceCandidate candidate) {
      String msg = encoder.convert({
        "RegisterICECandidate": {
          "candidate": candidate.candidate,
          "sdp_mid": candidate.sdpMid,
          "sdp_m_line_index": candidate.sdpMLineIndex,
          "to_user_id": userId,
        }
      });
      channel.sink.add(msg);
    };

    peerConnection?.onAddStream = (stream) {
      remoteStream = stream;
      remoteRenderer.srcObject = remoteStream;
    };

    peerConnection?.onTrack = (event) {
      remoteStream = event.streams[0];
      event.streams[0].getTracks().forEach((track) {
        remoteStream?.addTrack(track);
      });
      remoteRenderer.srcObject = remoteStream;
    };
  }

  sendOffer(WebSocketChannel channel) async {
    RTCSessionDescription offerSdp = await peerConnection!.createOffer();
    await peerConnection?.setLocalDescription(offerSdp);

    String msg = encoder.convert({
      "SDPOffer": {'sdp': offerSdp.sdp, "to_user_id": userId}
    });
    channel.sink.add(msg);
  }

  answerSdp(String sdpOffer, WebSocketChannel channel) async {
    RTCSessionDescription answerSdp = await peerConnection!.createAnswer();
    await peerConnection?.setLocalDescription(answerSdp);

    peerConnection
        ?.setRemoteDescription(RTCSessionDescription(sdpOffer, "offer"));

    String msg = encoder.convert({
      "SDPAnswer": {'sdp': answerSdp.sdp, "to_user_id": userId}
    });
    channel.sink.add(msg);

    isRemoteSet = true;
  }

  setRemoteAnswer(String sdpAnswer) {
    peerConnection
        ?.setRemoteDescription(RTCSessionDescription(sdpAnswer, "answer"));
    isRemoteSet = true;
  }

  addIceCandidate(ICECandidate iceCandidate) {
    peerConnection!.addCandidate(RTCIceCandidate(iceCandidate.candidate,
        iceCandidate.sdp_mid, iceCandidate.sdp_m_line_index));
    iceCandidates.add(iceCandidate);
  }
}
