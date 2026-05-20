import React from 'react'
import { ArrowRight } from 'lucide-react'

const PRODUCTS = [
  {
    id: 'D',
    name: 'Devist',
    color: '#0ea5e9',
    url: 'https://devist.net',
    tagline: '개발 워크플로우 자동화 CLI',
    description: '프로젝트 시작부터 배포까지. 개발 중 발생하는 반복 작업을 자동화합니다.',
    cmds: ['devist start my-app', 'devist log "버그 수정 완료"'],
    current: false,
  },
  {
    id: 'O',
    name: 'Operatist',
    color: '#f97316',
    url: 'https://operatist.net',
    tagline: '사업 운영 통합 관리 CLI',
    description: '거래, 계약, 정산까지. 사업자의 반복 운영 잡무를 CLI 한 줄로 처리합니다.',
    cmds: ['operatist tx list --today', 'operatist report monthly'],
    current: true,
  },
  {
    id: 'C',
    name: 'Curatist',
    color: '#8b5cf6',
    url: 'https://curatist.net',
    tagline: '일상 큐레이션 개인 비서 CLI',
    description: '이메일, 일정, 소비를 하나의 터미널에서. 중요한 것만 골라내는 개인 큐레이터입니다.',
    cmds: ['curatist mail inbox --priority', 'curatist schedule today'],
    current: false,
  },
]

export const Ecosystem: React.FC = () => (
  <section className="py-32 px-6">
    <div className="max-w-6xl mx-auto space-y-14">

      <div className="text-center space-y-4">
        <p className="text-sm uppercase tracking-[0.15em] font-medium text-zinc-500">Asurada System</p>
        <h2 className="text-3xl md:text-4xl font-bold text-zinc-50" style={{ wordBreak: 'keep-all' }}>
          같은 뇌, 다른 역할
        </h2>
        <p className="text-zinc-500 text-base max-w-xl mx-auto" style={{ wordBreak: 'keep-all' }}>
          Devist · Operatist · Curatist는 Asurada를 공유하는 자매 CLI입니다.
          각자의 영역에서 당신의 반복 작업을 대신합니다.
        </p>
      </div>

      <div className="grid md:grid-cols-3 gap-5">
        {PRODUCTS.map((p) => (
          <div
            key={p.id}
            className="rounded-2xl p-6 border space-y-5 flex flex-col"
            style={{
              background: p.current ? `${p.color}10` : 'rgba(255,255,255,0.02)',
              borderColor: p.current ? `${p.color}33` : 'rgba(255,255,255,0.06)',
            }}
          >
            <div className="flex items-center gap-3">
              <div
                className="w-8 h-8 rounded-full flex items-center justify-center text-xs font-bold flex-shrink-0"
                style={{ background: `${p.color}22`, color: p.color }}
              >
                {p.id}
              </div>
              <div>
                <p className="font-semibold text-zinc-100 text-sm">{p.name}</p>
                <p className="text-[11px] text-zinc-500">{p.tagline}</p>
              </div>
              {p.current && (
                <span className="ml-auto text-[10px] font-medium px-2 py-0.5 rounded-full" style={{ background: `${p.color}22`, color: p.color }}>
                  현재
                </span>
              )}
            </div>

            <p className="text-zinc-500 text-sm leading-relaxed flex-1" style={{ wordBreak: 'keep-all' }}>
              {p.description}
            </p>

            <div className="space-y-1.5">
              {p.cmds.map((cmd) => (
                <div key={cmd} className="flex items-center gap-2 bg-zinc-900/60 border border-zinc-800/40 rounded-lg px-3 py-1.5 font-mono text-xs">
                  <span className="text-zinc-700">$</span>
                  <span className="text-zinc-400">{cmd}</span>
                </div>
              ))}
            </div>

            {!p.current && (
              <a
                href={p.url}
                className="flex items-center gap-1.5 text-xs font-medium transition-colors duration-200"
                style={{ color: p.color }}
              >
                자세히 보기
                <ArrowRight size={12} />
              </a>
            )}
          </div>
        ))}
      </div>

    </div>
  </section>
)
