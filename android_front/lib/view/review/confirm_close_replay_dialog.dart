import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';

class ConfirmCloseReplayDialog extends StatefulWidget {

  @override
  _ConfirmCloseReplayDialogState createState() => _ConfirmCloseReplayDialogState();
}

class _ConfirmCloseReplayDialogState extends State<ConfirmCloseReplayDialog> {

  @override
  Widget build(BuildContext context) {
    return AlertDialog(
      backgroundColor: const Color(0xFF21262B),
      title: const Text(
        "Do you want to leave replay ?",
        style: TextStyle(
          color: Colors.white,
        ),
      ),
      actions: [
        TextButton(
          onPressed: () {
            Navigator.of(context).pop(false);
          },
          child: const Text(
            "Stay",
            style: TextStyle(
              color: Color(0xFF626af7),
            ),
          ),
        ),
        TextButton(
          onPressed: () {
            Navigator.of(context).pop(true);
          },
          child: const Text(
            "Leave",
            style: TextStyle(
              color: Color(0xFF626af7),
            ),
          ),
        ),
      ],
    );
  }
}