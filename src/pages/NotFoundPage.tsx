import { Link } from 'react-router-dom'

export function NotFoundPage() {
  return (
    <div className="page-stack">
      <div className="stacked-alert stacked-alert--danger">
        <h2>页面不存在</h2>
        <p>当前路由没有匹配到首版页面，请返回首页重新进入。</p>
        <div className="footer-actions">
          <Link className="button button--primary" to="/">
            返回首页
          </Link>
        </div>
      </div>
    </div>
  )
}
