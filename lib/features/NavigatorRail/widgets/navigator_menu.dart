import 'package:flutter/material.dart';

class NavigatorMenu extends StatelessWidget {
  const NavigatorMenu({super.key});

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.all(24.0),
      child: Column(
        mainAxisAlignment: MainAxisAlignment.spaceEvenly,
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          //TODO add movie series and collection menu switch
          ElevatedButton(onPressed: null, child: Text("Movies")),
          SizedBox(height: 12),
          ElevatedButton(onPressed: null, child: Text("Series")),
          SizedBox(height: 12),
          ElevatedButton(onPressed: null, child: Text("Collections")),
        ],
      ),
    );
  }
}
