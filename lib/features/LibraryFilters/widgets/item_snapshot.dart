import 'package:flutter/material.dart';

class ItemSnapshot extends StatelessWidget {
  final String name;
  final double width;
  final double height;

  const ItemSnapshot({
    super.key,
    required this.name,
    required this.width,
    required this.height,
  });

  @override
  Widget build(BuildContext context) {
    return Container(
      width: width,
      height: height,
      decoration: BoxDecoration(
        color: Color.fromARGB(255, 200, 220, 220),
        borderRadius: BorderRadius.all(Radius.circular(20)),
      ),
      child: Center(child: Text(name)),
    );
  }
}
