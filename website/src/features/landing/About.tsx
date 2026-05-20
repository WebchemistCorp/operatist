import React from 'react'
import { Brain, MessageSquare, Zap } from 'lucide-react'

const PRIMARY = '#2563eb'
const ACCENT = '#10b981'

const ASURADA_POINTS = [
  { icon: Brain, text: '사용자의 사업 맥락과 선호를 기억하고 점점 더 잘 이해합니다.' },
  { icon: MessageSquare, text: '자연어로 말하면 Asurada가 의도를 파악해서 알아서 처리합니다.' },
  { icon: Zap, text: '반복 작업을 패턴으로 학습해 다음엔 더 빠르게 자동 처리합니다.' },
]

const COMPARE_ITEMS = [
  {
    label: '기존 방식',
    points: [
      '세무, 법무, 행정 각각 다른 전문가에게 연락',
      '반복 서류 작업에 매주 수시간 소모',
      '지원사업 정보를 직접 찾아다녀야 함',
    ],
    accent: '#ef4444',
    dim: true,
  },
  {
    label: 'Workspace',
    points: [
      '자연어 한 마디로 전 영역 처리',
      '반복 업무 자동화로 본업 집중 시간 확보',
      '관련 지원사업을 AI가 알아서 탐색 & 신청',
    ],
    accent: ACCENT,
    dim: false,
  },
]

export const About: React.FC = () => (
  <section id="about" className="py-32 px-6">
    <div className="max-w-6xl mx-auto space-y-24">

      <div className="text-center space-y-6">
        <p className="text-sm uppercase tracking-[0.15em] font-medium" style={{ color: ACCENT }}>About Workspace</p>
        <h2 className="text-4xl md:text-5xl font-bold text-slate-50 leading-tight" style={{ wordBreak: 'keep-all' }}>
          사장님의 <span style={{ color: '#60a5fa' }}>AI 비서팀</span>
        </h2>
        <p className="text-slate-400 text-lg md:text-xl max-w-2xl mx-auto leading-relaxed" style={{ wordBreak: 'keep-all' }}>
          개인·법인 사업자가 혼자 감당하던 모든 잡일을 AI가 대신합니다.
          <br />
          당신은 오직 본업, 비전, 그리고 성장에만 집중하세요.
        </p>
      </div>

      <div className="grid md:grid-cols-2 gap-6">
        {COMPARE_ITEMS.map((item) => (
          <div
            key={item.label}
            className="rounded-2xl p-8 border space-y-5"
            style={{
              background: item.dim ? 'rgba(255,255,255,0.02)' : `${item.accent}0d`,
              borderColor: item.dim ? 'rgba(255,255,255,0.06)' : `${item.accent}33`,
            }}
          >
            <p className="font-semibold text-lg" style={{ color: item.dim ? '#f1f5f9' : item.accent }}>
              {item.label}
            </p>
            <ul className="space-y-3">
              {item.points.map((p) => (
                <li key={p} className="flex items-start gap-3 text-sm text-slate-400">
                  <span
                    className="mt-1.5 w-1.5 h-1.5 rounded-full flex-shrink-0"
                    style={{ background: item.dim ? '#ef4444' : item.accent }}
                  />
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
            <p className="text-xs uppercase tracking-[0.15em] font-medium" style={{ color: ACCENT }}>Core Engine</p>
            <h3 className="text-3xl font-bold text-slate-50">Asurada 연동</h3>
            <p className="text-slate-400 leading-relaxed" style={{ wordBreak: 'keep-all' }}>
              Workspace의 두뇌는 Asurada — AI 메모리 데몬입니다.
              사용자의 사업 맥락, 거래처, 선호 방식을 장기 기억하며
              점점 더 정확하고 빠르게 업무를 처리합니다.
            </p>
          </div>
          <ul className="space-y-4">
            {ASURADA_POINTS.map(({ icon: Icon, text }) => (
              <li key={text} className="flex items-start gap-4">
                <span
                  className="w-9 h-9 rounded-xl flex items-center justify-center flex-shrink-0"
                  style={{ background: `${ACCENT}1a` }}
                >
                  <Icon size={16} style={{ color: ACCENT }} />
                </span>
                <p className="text-slate-400 text-sm leading-relaxed pt-1.5" style={{ wordBreak: 'keep-all' }}>{text}</p>
              </li>
            ))}
          </ul>
        </div>
      </div>

    </div>
  </section>
)
