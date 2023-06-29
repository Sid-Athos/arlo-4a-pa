import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:miku/api/user/request/change_password_request.dart';
import 'package:shared_preferences/shared_preferences.dart';

import '../../api/user/api_user.dart';
import '../../model/user_model.dart';

class ChangePasswordDialog extends StatefulWidget {
  const ChangePasswordDialog({super.key});

  @override
  _ChangePasswordDialogState createState() => _ChangePasswordDialogState();
}

class _ChangePasswordDialogState extends State<ChangePasswordDialog> {
  final GlobalKey<FormState> _formKey = GlobalKey<FormState>();
  final oldPasswordController = TextEditingController();
  final newPasswordController = TextEditingController();
  final newPasswordConfirmController = TextEditingController();
  String errorMessage = "";

  String? _textEmptyValidator(String? value) {
    if (value == null || value.isEmpty) {
      return 'Please, fill this input';
    }
    return null;
  }

  String? _passwordValidator(String? value) {
    if (value == null || value.isEmpty) {
      return 'Please, fill this input';
    }
    if (value.length < 8) {
      return 'Password must contain at least 8 characters';
    }
    const pattern = r'^(?=.*?[A-Z])(?=.*?[0-9])(?=.*?[!@#\$&*~]).{8,}$';
    final regExp = RegExp(pattern);
    if (!regExp.hasMatch(value)) {
      return 'Rules : 1 capital letter, 1 digit, 1 special character';
    }
    return null;
  }

  String? _passwordConfirmValidator(String? value) {
    if (value == null || value.isEmpty) {
      return 'Please, fill this input';
    }
    if (newPasswordController.text != value) {
      return 'Must be the same that the password';
    }
    return null;
  }

  void sendRequest() async {
    if (_formKey.currentState!.validate()) {

      ChangePasswordRequest changePasswordRequest = ChangePasswordRequest(
          old_password: oldPasswordController.text,
          new_password: newPasswordController.text
      );

      SharedPreferences prefs = await SharedPreferences.getInstance();
      User? user = await ApiUser.changePassword(changePasswordRequest, prefs.getString("login_token")!);
      if (user == null) {
        setState(() {
          errorMessage = "Wrong password";
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
              controller: oldPasswordController,
              decoration: const InputDecoration(
                enabledBorder: UnderlineInputBorder(
                  borderSide: BorderSide(color: Colors.white),
                ),
                labelText: 'Enter your actual password',
                labelStyle: TextStyle(color: Colors.white),
              ),
              obscureText: true,
              style: const TextStyle(color: Colors.white),
            ),
            TextFormField(
              validator: _passwordValidator,
              controller: newPasswordController,
              decoration: const InputDecoration(
                enabledBorder: UnderlineInputBorder(
                  borderSide: BorderSide(color: Colors.white),
                ),
                labelText: 'Enter your new password',
                labelStyle: TextStyle(color: Colors.white),
              ),
              obscureText: true,
              style: const TextStyle(color: Colors.white),
            ),
            TextFormField(
              validator: _passwordConfirmValidator,
              controller: newPasswordConfirmController,
              decoration: const InputDecoration(
                enabledBorder: UnderlineInputBorder(
                  borderSide: BorderSide(color: Colors.white),
                ),
                labelText: 'Confirm your new password',
                labelStyle: TextStyle(color: Colors.white),
              ),
              obscureText: true,
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
