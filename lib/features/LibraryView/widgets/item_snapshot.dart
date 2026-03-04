import 'package:flutter/material.dart';

class ItemSnapshot extends StatelessWidget {
  final String name;

  const ItemSnapshot({super.key, required this.name});

  @override
  Widget build(BuildContext context) {
    return Card(child: Center(child: Text(name)));
  }
}
