import React from 'react'
import {
  Wallet,
  FileText,
  Calendar,
  Gift,
  Settings,
  Users,
} from 'lucide-react'

const PRIMARY = '#2563eb'
const ACCENT = '#10b981'

interface FeatureProps {
  icon: React.ElementType
  title: string
  desc: string
  highlight?: boolean
  color?: string
}

const FEATURES: FeatureProps[] = [
  {
    icon: Wallet,
    title: '자금관리',
    desc: '매출·지출 자동 집계, 현금흐름 분석, 세무 보고서 생성까지. 숫자에 매달리는 시간을 없앱니다.',
    highlight: true,
    color: PRIMARY,
  },
  {
    icon: FileText,
    title: '문서관리',
    desc: '계약서, 견적서, 사업계획서를 자연어 한 마디로 초안 작성. 법무 검토 포인트도 함께 짚어드립니다.',
    highlight: true,
    color: PRIMARY,
  },
  {
    icon: Calendar,
    title: '일정조율',
    desc: '미팅 조율, 데드라인 관리, 거래처 팔로업 리마인더. 놓치는 일정이 없도록 대신 챙깁니다.',
    color: ACCENT,
  },
  {
    icon: Gift,
    title: '지원사업 신청',
    desc: '정부·지자체 지원사업을 자동 탐색하고, 자격 요건 분석부터 신청서 작성까지 원스톱 지원.',
    color: ACCENT,
  },
  {
    icon: Settings,
    title: '운영 실무',
    desc: '재고 관리, 직원 급여 계산, 4대보험 신고 준비 등 반복 운영 업무를 자동화합니다.',
    color: PRIMARY,
  },
  {
    icon: Users,
    title: '거래처 관리',
    desc: '거래처 히스토리, 미수금 현황, 소통 이력을 통합 관리. 중요한 관계를 놓치지 않습니다.',
    color: ACCENT,
  },
]

export const Features: React.FC = () => (
  <section id="features" className="py-32 px-6">
    <div className="max-w-6xl mx-auto space-y-16">

      <div className="text-center space-y-4">
        <p className="text-sm uppercase tracking-[0.15em] font-medium" style={{ color: ACCENT }}>핵심 기능</p>
        <h2 className="text-4xl md:text-5xl font-bold text-slate-50" style={{ wordBreak: 'keep-all' }}>
          사업 전반을 커버하는
          <br />
          <span style={{ color: '#60a5fa' }}>6가지 핵심 기능</span>
        </h2>
        <p className="text-slate-400 text-lg max-w-xl mx-auto" style={{ wordBreak: 'keep-all' }}>
          한 명이 다 해야 했던 일들을 Workspace가 나눠 처리합니다.
        </p>
      </div>

      <div className="grid md:grid-cols-2 lg:grid-cols-3 gap-5">
        {FEATURES.map(({ icon: Icon, title, desc, highlight, color = PRIMARY }) => (
          <div
            key={title}
            className="rounded-2xl p-7 border space-y-4 transition-all duration-300 hover:scale-[1.01]"
            style={{
              background: highlight ? `${color}0d` : 'rgba(255,255,255,0.02)',
              borderColor: highlight ? `${color}33` : 'rgba(255,255,255,0.06)',
            }}
          >
            <span
              className="w-10 h-10 rounded-xl flex items-center justify-center"
              style={{ background: highlight ? `${color}22` : 'rgba(255,255,255,0.05)' }}
            >
              <Icon size={18} style={{ color: highlight ? color : '#64748b' }} />
            </span>
            <div className="space-y-2">
              <p className="font-semibold text-slate-100">{title}</p>
              <p className="text-slate-500 text-sm leading-relaxed" style={{ wordBreak: 'keep-all' }}>{desc}</p>
            </div>
          </div>
        ))}
      </div>

    </div>
  </section>
)
