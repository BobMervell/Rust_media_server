import 'package:flutter/material.dart';
import 'package:fluster_media_center/features/NavigatorRail/widgets/navigator_top_bar.dart';
import 'package:fluster_media_center/features/NavigatorRail/widgets/navigator_menu.dart';

class NavigatorDock extends StatelessWidget {
  const NavigatorDock({super.key});

  @override
  Widget build(BuildContext context) {
    return Container(
      decoration: BoxDecoration(
        color: Color.fromARGB(255, 180, 200, 200),
        borderRadius: BorderRadius.only(
          topRight: Radius.circular(12),
          bottomRight: Radius.circular(12),
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
