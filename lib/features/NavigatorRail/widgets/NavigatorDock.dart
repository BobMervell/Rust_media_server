import 'package:flutter/material.dart';
import 'package:fluster_media_center/features/NavigatorRail/widgets/NavigatorTopBar.dart';
import 'package:fluster_media_center/features/NavigatorRail/widgets/NavigatorMenu.dart';

class NavigatorDock extends StatelessWidget {
  const NavigatorDock({super.key});

  @override
  Widget build(BuildContext context) {
    return Container(
      decoration: BoxDecoration(
        color: Color.fromARGB(255, 180, 200, 200),
        borderRadius: BorderRadius.only(
          topRight: Radius.circular(20),
          bottomRight: Radius.circular(20),
        ),
      ),
      child: Column(
        children: [
          NavigatorTopBar(),
          Spacer(flex: 2),
          NavigatorMenu(),
          Spacer(flex: 2),
          NavigatorTopBar(),
        ],
      ),
    );
  }
}
