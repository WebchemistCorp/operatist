import React from 'react'
import { Brain, Shield, Zap } from 'lucide-react'

const PRIMARY = '#f97316'

const DIFF_ITEMS = [
  {
    label: '기존 방식',
    points: ['엑셀 여러 파일에 분산 관리', '수동 업데이트로 데이터 불일치', '맥락 없이 매번 처음부터 파악'],
    accent: '#ef4444',
    dim: true,
  },
  {
    label: 'Operatist',
    points: ['하나의 CLI로 사업 전반 통합 관리', 'Asurada가 운영 맥락을 기억', '터미널에서 즉시 조회·기록·분석'],
    accent: PRIMARY,
    dim: false,
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
          <span style={{ color: PRIMARY }}>Operat</span>e + As<span style={{ color: PRIMARY }}>ist</span>
        </h2>
        <p className="text-zinc-400 text-lg md:text-xl max-w-2xl mx-auto leading-relaxed" style={{ wordBreak: 'keep-all' }}>
          사업 운영을 CLI로 처리하는 도구.
          <br />
          Operatist는 사업자가 본업에 집중할 수 있도록 운영 잡무를 대신합니다.
        </p>
      </div>

      <div className="grid md:grid-cols-2 gap-6">
        {DIFF_ITEMS.map((item) => (
          <div
            key={item.label}
            className="rounded-2xl p-8 border space-y-5"
            style={{
              background: item.dim ? 'rgba(255,255,255,0.02)' : `${item.accent}0d`,
              borderColor: item.dim ? 'rgba(255,255,255,0.06)' : `${item.accent}33`,
            }}
          >
            <p className="font-semibold text-lg" style={{ color: item.dim ? '#ffffff' : item.accent }}>
              {item.label}
            </p>
            <ul className="space-y-3">
              {item.points.map((p) => (
                <li key={p} className="flex items-start gap-3 text-sm text-zinc-400">
                  <span className="mt-1.5 w-1.5 h-1.5 rounded-full flex-shrink-0" style={{ background: item.dim ? '#ef4444' : item.accent }} />
                  {p}
                </li>
              ))}
            </ul>
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
