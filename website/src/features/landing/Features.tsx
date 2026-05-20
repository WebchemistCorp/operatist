import React from 'react'
import { BarChart2, BookOpen, Briefcase, Calendar, ClipboardList, DollarSign, FileText, Users } from 'lucide-react'

const PRIMARY = '#f97316'

interface FeatureProps {
  icon: React.ElementType
  title: string
  desc: string
  highlight?: boolean
}

const FEATURES: FeatureProps[] = [
  {
    icon: Briefcase,
    title: '자산 관리',
    desc: '노트북, 차량, 장비 등 회사 자산을 등록하고 상태를 추적합니다.',
    highlight: true,
  },
  {
    icon: BarChart2,
    title: '자금 관리',
    desc: '수입·지출을 기록하고 월별 현금흐름을 한눈에 요약합니다.',
    highlight: true,
  },
  {
    icon: FileText,
    title: '문서 관리',
    desc: '계약서, 제안서, 보고서를 보관하고 파일을 안전하게 저장합니다.',
  },
  {
    icon: Users,
    title: '거래처 / 연락처',
    desc: '고객사, 파트너사, 프리랜서 등 사업 관계를 체계적으로 관리합니다.',
  },
  {
    icon: ClipboardList,
    title: '구독 관리',
    desc: 'SaaS, 월정액 서비스의 결제일과 금액을 놓치지 않고 파악합니다.',
  },
  {
    icon: Calendar,
    title: '일정 관리',
    desc: '미팅, 마감일, 납부일 등 사업 일정을 CLI에서 바로 조회합니다.',
  },
  {
    icon: DollarSign,
    title: '지원사업 신청',
    desc: '정부 지원사업을 발굴하고 신청 단계별 진행 현황을 추적합니다.',
  },
  {
    icon: BookOpen,
    title: '태스크 관리',
    desc: '운영 할일을 우선순위별로 정리하고 완료 현황을 파악합니다.',
  },
]

export const Features: React.FC = () => (
  <section id="features" className="py-32 px-6">
    <div className="max-w-6xl mx-auto space-y-16">

      <div className="text-center space-y-4">
        <p className="text-sm uppercase tracking-[0.15em] font-medium" style={{ color: PRIMARY }}>Features</p>
        <h2 className="text-4xl md:text-5xl font-bold text-zinc-50" style={{ wordBreak: 'keep-all' }}>
          사업 운영의 전 영역을 커버
        </h2>
        <p className="text-zinc-400 text-lg max-w-xl mx-auto" style={{ wordBreak: 'keep-all' }}>
          하나의 CLI로 사업자가 관리해야 하는 모든 영역을 처리합니다.
        </p>
      </div>

      <div className="grid md:grid-cols-2 lg:grid-cols-4 gap-5">
        {FEATURES.map(({ icon: Icon, title, desc, highlight }) => (
          <div
            key={title}
            className="rounded-2xl p-6 border space-y-4 transition-all duration-300 hover:scale-[1.01]"
            style={{
              background: highlight ? `${PRIMARY}0d` : 'rgba(255,255,255,0.02)',
              borderColor: highlight ? `${PRIMARY}33` : 'rgba(255,255,255,0.06)',
            }}
          >
            <span
              className="w-10 h-10 rounded-xl flex items-center justify-center"
              style={{ background: highlight ? `${PRIMARY}22` : 'rgba(255,255,255,0.05)' }}
            >
              <Icon size={18} style={{ color: highlight ? PRIMARY : '#71717a' }} />
            </span>
            <div className="space-y-2">
              <p className="font-semibold text-zinc-100 text-sm">{title}</p>
              <p className="text-zinc-500 text-xs leading-relaxed" style={{ wordBreak: 'keep-all' }}>{desc}</p>
            </div>
          </div>
        ))}
      </div>

    </div>
  </section>
)
