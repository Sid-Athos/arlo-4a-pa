import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:miku/api/user/request/change_password_request.dart';
import 'package:miku/api/user/request/update_user_request.dart';
import 'package:shared_preferences/shared_preferences.dart';

import '../../api/user/api_user.dart';
import '../../model/user/user_model.dart';

class ChangePseudoDialog extends StatefulWidget {
  const ChangePseudoDialog({super.key});

  @override
  _ChangePseudoDialogState createState() => _ChangePseudoDialogState();
}

class _ChangePseudoDialogState extends State<ChangePseudoDialog> {
  final GlobalKey<FormState> _formKey = GlobalKey<FormState>();
  final pseudoController = TextEditingController();
  String errorMessage = "";

  String? _textEmptyValidator(String? value) {
    if (value == null || value.isEmpty) {
      return 'Please, fill this input';
    }
    return null;
  }

  void sendRequest() async {
    if (_formKey.currentState!.validate()) {

      UpdateUserRequest updateUserRequest = UpdateUserRequest(
          pseudo: pseudoController.text,
      );

      SharedPreferences prefs = await SharedPreferences.getInstance();
      User? user = await ApiUser.updateUser(updateUserRequest, prefs.getString("login_token")!);

      if (user == null) {
        setState(() {
          errorMessage = "Pseudo Already Used By Another Player";
        });
      } else {
        Navigator.pop(context);
      }
    }
  }

  @override
  Widget build(BuildContext context) {
    return AlertDialog(
      backgroundColor: const Color(0xFF21262B),
      title: const Text(
        "Change Password",
        style: TextStyle(
          color: Colors.white,
        ),
      ),
      content: Form(
        key: _formKey,
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            TextFormField(
              validator: _textEmptyValidator,
              controller: pseudoController,
              decoration: const InputDecoration(
                enabledBorder: UnderlineInputBorder(
                  borderSide: BorderSide(color: Colors.white),
                ),
                labelText: 'New Pseudo',
                labelStyle: TextStyle(color: Colors.white),
              ),
              style: const TextStyle(color: Colors.white),
            ),
            Padding(
              padding: const EdgeInsets.only(top: 20),
              child: Text(
                  errorMessage,
                  style: const TextStyle(color: Colors.red)
              ),
            ),
          ],
        ),
      ),
      actions: [
        TextButton(
          onPressed: () {
            Navigator.of(context).pop();
          },
          child: const Text(
            "Cancel",
            style: TextStyle(
              color: Color(0xFF626af7),
            ),
          ),
        ),
        TextButton(
          onPressed: sendRequest,
          child: const Text(
            "Update",
            style: TextStyle(
              color: Color(0xFF626af7),
            ),
          ),
        ),
      ],
    );
  }
}
