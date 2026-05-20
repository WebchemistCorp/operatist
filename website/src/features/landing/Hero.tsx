import React from 'react'
import { ArrowDown, Briefcase } from 'lucide-react'
import { Button } from '@/components/ui/button'

const PRIMARY = '#2563eb'
const ACCENT = '#10b981'
const BG = '#0a0f1e'

export const Hero: React.FC = () => (
  <section className="relative min-h-[100dvh] flex flex-col justify-center overflow-hidden">
    <div className="absolute inset-0 pointer-events-none">
      <div className="orb absolute rounded-full" style={{ width: 700, height: 700, background: PRIMARY, opacity: 0.2, filter: 'blur(120px)', top: '-20%', right: '-15%' }} />
      <div className="orb absolute rounded-full" style={{ width: 500, height: 500, background: ACCENT, opacity: 0.15, filter: 'blur(100px)', bottom: '-10%', left: '-10%', animationDelay: '3s' }} />
      <div className="float absolute rounded-full" style={{ width: 300, height: 300, background: PRIMARY, opacity: 0.1, filter: 'blur(80px)', top: '30%', left: '20%', animationDelay: '5s' }} />
      <div
        className="absolute inset-0"
        style={{
          backgroundImage: `linear-gradient(${PRIMARY}0d 1px, transparent 1px), linear-gradient(90deg, ${PRIMARY}0d 1px, transparent 1px)`,
          backgroundSize: '80px 80px',
        }}
      />
      <div className="absolute bottom-0 left-0 right-0 h-40" style={{ background: `linear-gradient(to top, ${BG}, transparent)` }} />
    </div>

    <div className="relative max-w-6xl mx-auto px-6 w-full pt-20 pb-16">
      <div className="flex flex-col items-center text-center space-y-8">
        <div
          className="reveal flex items-center gap-3 px-3 py-1.5 rounded-full"
          style={{ background: `${PRIMARY}1a`, border: `1px solid ${PRIMARY}33`, animationDelay: '100ms' }}
        >
          <Briefcase size={12} style={{ color: ACCENT }} />
          <span className="text-[11px] uppercase tracking-[0.15em] font-medium" style={{ color: ACCENT }}>
            AI 비즈니스 어시스턴트
          </span>
        </div>

        <h1
          className="reveal text-5xl md:text-7xl lg:text-8xl font-bold tracking-tight leading-[1.05] text-slate-50"
          style={{ animationDelay: '150ms', wordBreak: 'keep-all' }}
        >
          사업의 모든 잡일,
          <br />
          <span style={{ color: '#60a5fa' }}>AI에게 맡기세요</span>
        </h1>

        <p className="reveal text-slate-400 text-lg md:text-xl max-w-2xl leading-relaxed" style={{ animationDelay: '200ms', wordBreak: 'keep-all' }}>
          자금관리, 문서 작업, 법무 검토, 일정 조율, 지원사업 신청까지.
          <br />
          Workspace가 사업 전반을 대신 처리하니, 당신은 본업에만 집중하세요.
        </p>

        <div className="reveal flex items-center gap-3 flex-wrap justify-center" style={{ animationDelay: '250ms' }}>
          <Button asChild className="group rounded-full px-8 py-4 h-auto bg-blue-600 hover:bg-blue-500 text-white font-semibold text-sm shadow-[0_0_30px_rgba(37,99,235,0.3)] hover:scale-[1.02] active:scale-[0.98] transition-all duration-500">
            <a href="#cta">
              무료로 시작하기
              <span className="w-6 h-6 rounded-full flex items-center justify-center group-hover:translate-y-0.5 transition-transform bg-black/15">
                <ArrowDown size={12} />
              </span>
            </a>
          </Button>
          <Button asChild variant="ghost" className="rounded-full px-6 py-4 h-auto border border-slate-700/60 text-slate-300 hover:text-slate-100 hover:border-blue-500/40 hover:bg-blue-500/5 transition-all duration-500">
            <a href="#features">기능 보기</a>
          </Button>
        </div>

        <div className="reveal w-full max-w-2xl mt-4" style={{ animationDelay: '350ms' }}>
          <div className="p-1.5 rounded-2xl bg-slate-800/40 border border-slate-700/40">
            <div className="p-4 bg-slate-900/80 border border-slate-800/60 rounded-xl font-mono text-sm text-left">
              <div className="flex items-center gap-2 mb-3">
                <span className="w-2.5 h-2.5 rounded-full bg-red-500/60" />
                <span className="w-2.5 h-2.5 rounded-full bg-yellow-500/60" />
                <span className="w-2.5 h-2.5 rounded-full bg-green-500/60" />
              </div>
              <p className="text-slate-600">나 &gt; <span className="text-slate-300">이번 달 매출 정리해서 세무사한테 보낼 자료 만들어줘</span></p>
              <p className="mt-1" style={{ color: ACCENT }}>Workspace &gt; 5월 거래 내역 집계 완료. PDF 보고서 생성 중...</p>
              <p className="text-slate-600 mt-2">나 &gt; <span className="text-slate-300">소기업 지원사업 신청서도 작성해줘</span></p>
              <p className="mt-1" style={{ color: '#60a5fa' }}>Workspace &gt; 중소벤처기업부 2026년 소기업 성장지원 사업에 적합합니다. 신청서 초안을 작성했습니다.</p>
            </div>
          </div>
        </div>

        <div className="reveal flex items-center gap-8 pt-4" style={{ animationDelay: '400ms' }}>
          {[
            { label: '웹앱', desc: '브라우저에서 바로' },
            { label: '모바일', desc: 'iOS · Android' },
            { label: 'CLI', desc: '터미널 파워유저' },
          ].map((item) => (
            <div key={item.label} className="text-center">
              <p className="text-slate-200 text-sm font-semibold">{item.label}</p>
              <p className="text-slate-600 text-xs mt-0.5">{item.desc}</p>
            </div>
          ))}
        </div>
      </div>
    </div>
  </section>
)
