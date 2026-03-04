import 'package:flutter/material.dart';
import 'package:fluster_media_center/features/NavigatorRail/widgets/navigator_dock.dart';

class NavigatorRail extends StatelessWidget {
  const NavigatorRail({super.key});

  @override
  Widget build(BuildContext context) {
    return LayoutBuilder(
      builder: (context, constraints) {
        final maxHeight = constraints.maxHeight;

        final targetHeight = maxHeight / 2;

        //TODO tune or replace magic number for dock height
        final useFullHeight = maxHeight < 600;

        return Column(
          children: [
            if (!useFullHeight) const Spacer(),

            SizedBox(
              height: useFullHeight ? maxHeight : targetHeight,
              child: NavigatorDock(),
            ),

            if (!useFullHeight) const Spacer(),
          ],
        );
      },
    );
  }
}
