import React from 'react'

const ECOSYSTEM = [
  { id: 'D', name: 'Devist', color: '#0ea5e9', url: 'https://devist.net' },
  { id: 'O', name: 'Operatist', color: '#f97316', url: 'https://operatist.net' },
  { id: 'C', name: 'Curatist', color: '#8b5cf6', url: 'https://curatist.net' },
]

export const Footer: React.FC = () => (
  <footer className="border-t border-zinc-800/60 px-6 py-8">
    <div className="max-w-6xl mx-auto space-y-4">
      <div className="flex items-center gap-4 flex-wrap pb-2">
        <span className="text-[11px] text-zinc-600 font-mono">Asurada System</span>
        {ECOSYSTEM.map(e => (
          <a
            key={e.id}
            href={e.url}
            className="flex items-center gap-1.5 text-[11px] font-mono text-zinc-600 hover:text-zinc-400 transition-colors duration-200"
          >
            <span className="w-1.5 h-1.5 rounded-full" style={{ background: e.color }} />
            {e.name}
          </a>
        ))}
      </div>
      <div className="flex flex-wrap gap-x-6 gap-y-1 text-[11px] text-zinc-600 font-mono">
        <span>상호 Webchemist Corp</span>
        <span>대표 윤원열</span>
        <span>사업자등록번호 722-86-03469</span>
      </div>
      <div className="flex flex-wrap gap-x-6 gap-y-1 text-[11px] text-zinc-600 font-mono">
        <span>주소 경기도 안양시 만안구 병목안로 2, 11층 1103호-01A(안양동)</span>
        <span>이메일 info@webchemist.net</span>
      </div>
      <div className="flex items-center justify-between pt-2">
        <span className="font-mono text-[11px] text-zinc-700">© 2024–{new Date().getFullYear()} Webchemist Corp</span>
        <span className="font-mono text-[11px] text-zinc-700">Powered by Asurada</span>
      </div>
    </div>
  </footer>
)
