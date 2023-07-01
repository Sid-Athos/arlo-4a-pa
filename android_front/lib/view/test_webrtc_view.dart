import 'dart:convert';

import 'package:flutter/material.dart';
import 'package:flutter_webrtc/flutter_webrtc.dart';
import 'package:web_socket_channel/web_socket_channel.dart';
import 'package:web_socket_channel/io.dart';
import 'dart:developer' as developer;

class WebRTCSession extends StatefulWidget {
  @override
  _WebRTCSessionState createState() => _WebRTCSessionState();
}

class _WebRTCSessionState extends State<WebRTCSession> {

  RTCPeerConnection? _peerConnection;
  MediaStream? _localStream;
  MediaStream? _remoteStream;
  RTCVideoRenderer _localRenderer = RTCVideoRenderer();
  RTCVideoRenderer _remoteRenderer = RTCVideoRenderer();
  WebSocketChannel? _socketChannel;

  @override
  void initState() {
    super.initState();
    initRenderers();
    _connectWebSocket();
  }

  void _connectWebSocket() {
    final url = 'ws://192.168.137.117:7589/rtc';
    _socketChannel = IOWebSocketChannel.connect(url);
    _socketChannel?.stream.listen((event) async {
      if (_peerConnection == null) {
        final Map<String, dynamic> configuration = {
          'iceServers': [
            {'url': 'stun:stun.l.google.com:19302'},
          ],
        };

        _peerConnection = await createPeerConnection(configuration);

        JsonDecoder _decoder = JsonDecoder();
        Map<String, dynamic> data = _decoder.convert(event);

        _peerConnection?.setRemoteDescription(RTCSessionDescription(data['sdp'], data['type']));

        _peerConnection?.onAddStream = (stream) {
          developer.log("Stream ajouté");
          setState(() {
            _remoteStream = stream;
            _remoteRenderer.srcObject = stream;
          });
        };

        _localStream = await navigator.mediaDevices.getUserMedia({
          'audio': true,
          'video': true,
        });

        _localStream?.getTracks().forEach((track) {
          developer.log("Track ajouté");
          _peerConnection?.addTrack(track, _localStream!);
        });
      }
    });
  }

  initRenderers() async {

    await _remoteRenderer.initialize();
    await _localRenderer.initialize();

    try {
      final Map<String, dynamic> configuration = {
        'iceServers': [
          {'url': 'stun:stun.l.google.com:19302'},
        ],
      };

      _peerConnection = await createPeerConnection(configuration);

      _peerConnection?.onAddStream = (stream) {
        developer.log("Stream ajouté");
        setState(() {
          _remoteStream = stream;
          _remoteRenderer.srcObject = stream;
        });
      };

      _localStream = await navigator.mediaDevices.getUserMedia({
        'audio': true,
        'video': true,
      });

      _localStream?.getTracks().forEach((track) {
        developer.log("Track ajouté");
        _peerConnection?.addTrack(track, _localStream!);
      });

      final offerSdp = await _peerConnection?.createOffer();
      await _peerConnection?.setLocalDescription(offerSdp!);

      final encoder = JsonEncoder();
      String msg = encoder.convert({'sdp': offerSdp!.sdp, 'type': offerSdp.type});
      _socketChannel?.sink.add(msg);

    } catch (e) {
      print('Erreur lors de la création de la session WebRTC : $e');
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text('WebRTC Session'),
      ),
      body: Column(
        children: [
          Expanded(
            child: RTCVideoView(_remoteRenderer),
          ),
          ElevatedButton(
            onPressed: () async {
              _peerConnection?.close();
              _localStream?.dispose();
              _remoteStream?.dispose();
              _remoteRenderer = RTCVideoRenderer();
              await _remoteRenderer.initialize();
              _peerConnection = null;
            },
            child: Text('Terminer la session'),
          ),
        ],
      ),
    );
  }
}
