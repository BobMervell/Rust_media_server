// ignore: unused_import
import 'package:fluster_media_center/src/rust/api/media.dart';
import 'package:flutter/material.dart';

class PlaceholderBanner extends StatelessWidget {
  const PlaceholderBanner({super.key});

  @override
  Widget build(BuildContext context) {
    return Center(
      child: ElevatedButton(
        onPressed: () async {
          // await start(          );
        },
        child: Text("Have a nice time here"),
      ),
    );
  }
}
