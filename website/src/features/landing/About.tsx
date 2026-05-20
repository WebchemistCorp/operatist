import React from 'react'
import { Brain, Shield, Zap } from 'lucide-react'

const PRIMARY = '#f97316'

const SCENARIOS = [
  {
    situation: '월말 정산할 때',
    cmd: 'operatist report monthly',
    result: '매출 4,200,000 · 지출 1,850,000 · 순이익 2,350,000',
  },
  {
    situation: '오늘 입금 확인할 때',
    cmd: 'operatist tx list --today',
    result: '3건 · 총 1,250,000원 입금 확인',
  },
  {
    situation: '계약 만료 체크할 때',
    cmd: 'operatist contract list --expiring 30d',
    result: '2개 계약 30일 내 만료 예정',
  },
  {
    situation: '직원 급여 처리할 때',
    cmd: 'operatist payroll run --month 2025-05',
    result: '3명 · 총 9,000,000원 처리 완료',
  },
]

const ASURADA_POINTS = [
  { icon: Brain, text: '자산, 자금, 문서의 운영 맥락을 기억하고 학습' },
  { icon: Zap, text: '반복 패턴을 분석해 다음 운영 작업을 예측하고 제안' },
  { icon: Shield, text: 'Supabase 연동으로 데이터 안전하게 동기화' },
]

export const About: React.FC = () => (
  <section id="about" className="py-32 px-6">
    <div className="max-w-6xl mx-auto space-y-24">

      <div className="text-center space-y-6">
        <p className="text-sm uppercase tracking-[0.15em] font-medium" style={{ color: PRIMARY }}>About Operatist</p>
        <h2 className="text-4xl md:text-5xl font-bold text-zinc-50 leading-tight" style={{ wordBreak: 'keep-all' }}>
          사업 운영,<br />
          <span style={{ color: PRIMARY }}>터미널 하나로 끝냅니다</span>
        </h2>
        <p className="text-zinc-400 text-lg md:text-xl max-w-2xl mx-auto leading-relaxed" style={{ wordBreak: 'keep-all' }}>
          거래 기록, 계약 관리, 정산까지.<br />
          Operatist는 사업자의 반복 운영 잡무를 CLI 한 줄로 처리합니다.
        </p>
      </div>

      <div className="grid md:grid-cols-2 gap-4">
        {SCENARIOS.map(({ situation, cmd, result }) => (
          <div
            key={cmd}
            className="rounded-2xl p-6 border space-y-4"
            style={{ background: `${PRIMARY}08`, borderColor: `${PRIMARY}1a` }}
          >
            <p className="text-xs text-zinc-500 font-medium tracking-wide">{situation}</p>
            <div className="flex items-center gap-2 bg-zinc-900/80 border border-zinc-800/60 rounded-xl px-4 py-2.5 font-mono text-sm">
              <span className="text-zinc-600">$</span>
              <span className="text-zinc-200">{cmd}</span>
            </div>
            <p className="text-sm font-mono" style={{ color: PRIMARY }}>→ {result}</p>
          </div>
        ))}
      </div>

      <div className="rounded-2xl p-10 border" style={{ background: `${PRIMARY}08`, borderColor: `${PRIMARY}22` }}>
        <div className="grid md:grid-cols-2 gap-10 items-center">
          <div className="space-y-4">
            <p className="text-xs uppercase tracking-[0.15em] font-medium" style={{ color: PRIMARY }}>Core System</p>
            <h3 className="text-3xl font-bold text-zinc-50">Asurada System</h3>
            <p className="text-zinc-400 leading-relaxed" style={{ wordBreak: 'keep-all' }}>
              Operatist의 핵심 엔진. 자율학습 AI 시스템으로 사업 운영 맥락을 기억합니다.
              단순 기록 도구가 아닌, 사업자의 운영 패턴을 이해하는 동반자입니다.
            </p>
          </div>
          <ul className="space-y-4">
            {ASURADA_POINTS.map(({ icon: Icon, text }) => (
              <li key={text} className="flex items-start gap-4">
                <span className="w-9 h-9 rounded-xl flex items-center justify-center flex-shrink-0" style={{ background: `${PRIMARY}1a` }}>
                  <Icon size={16} style={{ color: PRIMARY }} />
                </span>
                <p className="text-zinc-400 text-sm leading-relaxed pt-1.5" style={{ wordBreak: 'keep-all' }}>{text}</p>
              </li>
            ))}
          </ul>
        </div>
      </div>

    </div>
  </section>
)
