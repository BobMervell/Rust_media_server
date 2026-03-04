import 'package:flutter/material.dart';

class NavigatorTopBar extends StatelessWidget {
  const NavigatorTopBar({super.key});

  @override
  Widget build(BuildContext context) {
    return const Row(
      mainAxisAlignment: MainAxisAlignment.spaceEvenly,
      children: [
        Icon(Icons.airlines),
        Text("Media Center"),
        IconButton(onPressed: null, icon: Icon(Icons.search)),
        //TODO add search functionality
      ],
    );
  }
}
