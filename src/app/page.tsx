'use client';

import { Game } from './game';

export default function Home() {
  return (
    <main className="flex h-screen flex-col items-center bg-blue-100 pt-10">
      <Game />
    </main>
  );
}
