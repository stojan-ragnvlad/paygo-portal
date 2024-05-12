import { Component, OnInit } from '@angular/core';
import { RouterOutlet } from '@angular/router';
// import init, { greet } from 'compile-time-tools/rust';
// import wasmData, { greet } from 'compile-time-tools/rust_bg.wasm';

@Component({
  selector: 'app-root',
  standalone: true,
  imports: [RouterOutlet],
  templateUrl: './app.component.html',
  styleUrl: './app.component.css'
})
export class AppComponent implements OnInit{
  title = 'web-browser-client-angular';

  ngOnInit(): void {
    fetch('rust_bg.wasm').then(response => response.arrayBuffer())
      .then(bytes => WebAssembly.instantiate(bytes))
      // @ts-ignore
      .then(results => results.instance.exports.greet());
  }
}
