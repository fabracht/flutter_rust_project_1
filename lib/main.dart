import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:root_access/root_access.dart';
import 'ffi.dart';

import 'dart:async';

void main() => runApp(new MyApp());

class MyApp extends StatefulWidget {
  @override
  _MyAppState createState() => new _MyAppState();
}

class _MyAppState extends State<MyApp> {
  bool _rootAccess = false;

  @override
  void initState() {
    super.initState();
    initRootRequest();
  }

  Future<void> initRootRequest() async {

    // bool rootAccess = await RootAccess.requestRootAccess;
    // bool rootAccess = false;
    var pingResults = await api.runPing();
    if (kDebugMode) {


      print("Ping Results ");


    }
    // If the widget was removed from the tree while the asynchronous platform
    // message was in flight, we want to discard the reply rather than calling
    // setState to update our non-existent appearance.
    if (!mounted) return;
    // var counterPlus = await api.addOne(number: _counter);
    // var sentBytes = await api.sendUdpMessage();



    setState(() {
      _rootAccess = false;
    });
  }

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: Scaffold(
        appBar: AppBar(
          title: const Text('Plugin example app'),
        ),
        body: Center(
          child: Text('Root access granted: $_rootAccess\n'),
        ),
      ),
    );
  }
}
