import { Component, OnInit } from '@angular/core';
import { RouterOutlet } from '@angular/router';
import init, { greet } from 'compile-time-tools/rust_bg.wasm';

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
    init().then(() => greet());
  }
}
