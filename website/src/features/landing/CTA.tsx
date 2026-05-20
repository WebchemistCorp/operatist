import React from 'react'
import { ArrowRight, Terminal } from 'lucide-react'
import { Button } from '@/components/ui/button'

const PRIMARY = '#f97316'

const INSTALL_STEPS = [
  { cmd: 'brew install webchemistcorp/tap/operatist', label: 'Homebrew (macOS / Linux)' },
  { cmd: 'operatist setup', label: 'Supabase 초기화' },
  { cmd: 'operatist --help', label: '시작하기' },
]

export const CTA: React.FC = () => (
  <section id="cta" className="py-32 px-6">
    <div className="max-w-4xl mx-auto space-y-16">

      <div className="text-center space-y-4">
        <p className="text-sm uppercase tracking-[0.15em] font-medium" style={{ color: PRIMARY }}>시작하기</p>
        <h2 className="text-4xl md:text-5xl font-bold text-zinc-50" style={{ wordBreak: 'keep-all' }}>
          지금 바로
          <br />
          <span style={{ color: PRIMARY }}>사업 운영을 단순하게</span>
        </h2>
        <p className="text-zinc-400 text-lg max-w-xl mx-auto" style={{ wordBreak: 'keep-all' }}>
          터미널 하나로 사업 전반을 관리하세요.
        </p>
      </div>

      <div className="rounded-2xl p-8 border space-y-5" style={{ background: `${PRIMARY}08`, borderColor: `${PRIMARY}22` }}>
        <div className="flex items-center gap-2 mb-2">
          <Terminal size={16} style={{ color: PRIMARY }} />
          <p className="text-sm font-semibold text-zinc-300">설치 방법</p>
        </div>
        <div className="space-y-3">
          {INSTALL_STEPS.map(({ cmd, label }, i) => (
            <div key={cmd} className="space-y-1">
              <p className="text-xs text-zinc-600 font-mono">{`# ${i + 1}. ${label}`}</p>
              <div className="flex items-center gap-3 bg-zinc-900/80 border border-zinc-800/60 rounded-xl px-4 py-3 font-mono text-sm">
                <span className="text-zinc-600">$</span>
                <span className="text-zinc-200">{cmd}</span>
              </div>
            </div>
          ))}
        </div>
      </div>

      <div className="flex flex-col sm:flex-row items-center justify-center gap-4">
        <Button asChild className="rounded-full px-8 py-4 h-auto font-semibold text-zinc-950 hover:scale-[1.02] active:scale-[0.98] transition-all duration-500" style={{ background: PRIMARY, boxShadow: `0 0 24px ${PRIMARY}33` }}>
          <a href="https://github.com/WebchemistCorp/operatist/releases" target="_blank" rel="noreferrer">
            GitHub Releases에서 다운로드
            <ArrowRight size={14} />
          </a>
        </Button>
        <Button asChild variant="ghost" className="rounded-full px-6 py-4 h-auto border border-zinc-700/60 text-zinc-400 hover:text-zinc-200 transition-all duration-500">
          <a href="https://github.com/WebchemistCorp/operatist" target="_blank" rel="noreferrer">
            소스코드 보기
          </a>
        </Button>
      </div>

    </div>
  </section>
)
