"use client";
import { useState } from "react";
export default function Home() {
  const [query, setQuery] = useState("");
  const [icons, setIcons] = useState<any[]>([]);
  const [copied, setCopied] = useState("");
  const search = async () => {
    const res = await fetch(`/api/icons?q=${encodeURIComponent(query)}`);
    const data = await res.json();
    setIcons(data.icons || []);
  };
  const copySvg = (svg: string, id: string) => {
    navigator.clipboard.writeText(svg);
    setCopied(id);
    setTimeout(() => setCopied(""), 2000);
  };
  return (
    <main className="min-h-screen bg-gradient-to-br from-teal-900 via-black to-emerald-900 text-white p-8">
      <div className="max-w-6xl mx-auto">
        <h1 className="text-5xl font-bold mb-4 bg-gradient-to-r from-teal-400 to-emerald-400 bg-clip-text text-transparent">svgrepo</h1>
        <p className="text-xl text-gray-300 mb-8">500k+ free SVG vectors & icons</p>
        <div className="bg-white/10 backdrop-blur-lg rounded-2xl p-8 mb-8 flex gap-4">
          <input value={query} onChange={(e) => setQuery(e.target.value)} placeholder="Search icons..."
            className="flex-1 bg-white/5 border border-white/20 rounded-xl px-4 py-3 text-white placeholder-gray-400 focus:outline-none focus:border-teal-400"
            onKeyDown={(e) => e.key === "Enter" && search()} />
          <button onClick={search}
            className="px-8 py-3 bg-gradient-to-r from-teal-600 to-emerald-600 rounded-xl font-semibold hover:opacity-90 transition">Search</button>
        </div>
        <div className="grid grid-cols-3 md:grid-cols-5 lg:grid-cols-8 gap-4">
          {icons.map((icon) => (
            <button key={icon.id} onClick={() => copySvg(icon.svg, icon.id)}
              className="bg-white/10 backdrop-blur rounded-xl p-4 hover:scale-110 transition cursor-pointer group">
              <div className="text-4xl mb-2" dangerouslySetInnerHTML={{ __html: icon.svg }} />
              <p className="text-xs text-gray-400 group-hover:text-white transition">{icon.name}</p>
              {copied === icon.id && <p className="text-xs text-emerald-400">Copied!</p>}
            </button>
          ))}
        </div>
      </div>
    </main>
  );
}