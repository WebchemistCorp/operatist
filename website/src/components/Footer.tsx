import React from 'react'

export const Footer: React.FC = () => (
  <footer className="border-t border-slate-800/60 px-6 py-8">
    <div className="max-w-6xl mx-auto space-y-4">
      <div className="flex flex-wrap gap-x-6 gap-y-1 text-[11px] text-slate-600 font-mono">
        <span>상호 Webchemist Corp</span>
        <span>대표 윤원열</span>
        <span>사업자등록번호 722-86-03469</span>
      </div>
      <div className="flex flex-wrap gap-x-6 gap-y-1 text-[11px] text-slate-600 font-mono">
        <span>주소 경기도 안양시 만안구 병목안로 2, 11층 1103호-01A(안양동)</span>
        <span>이메일 info@webchemist.net</span>
      </div>
      <div className="flex items-center justify-between pt-2">
        <span className="font-mono text-[11px] text-slate-700">© 2024–{new Date().getFullYear()} Webchemist Corp</span>
        <span className="font-mono text-[11px] text-slate-700">Powered by Asurada</span>
      </div>
    </div>
  </footer>
)
