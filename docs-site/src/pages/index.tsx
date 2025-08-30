import clsx from 'clsx';
import Link from '@docusaurus/Link';
import useDocusaurusContext from '@docusaurus/useDocusaurusContext';
import Layout from '@theme/Layout';
import HomepageFeatures from '@site/src/components/HomepageFeatures';
import PerformanceComparison from '@site/src/components/PerformanceComparison';
import Heading from '@theme/Heading';

import styles from './index.module.css';

function HomepageHeader() {
  const {siteConfig} = useDocusaurusContext();
  return (
    <header className={clsx('hero hero--primary', styles.heroBanner)}>
      <div className="container">
        <div className={styles.heroContent}>
          <div className={styles.heroText}>
            <Heading as="h1" className="hero__title">
              {siteConfig.title}
            </Heading>
            <p className="hero__subtitle">{siteConfig.tagline}</p>
            <div className={styles.performanceHighlight}>
              <span className={styles.highlightBadge}>🚀 NEW</span>
              <span className={styles.highlightText}>Up to 4x faster than traditional data processing</span>
            </div>
            <div className={styles.buttons}>
              <Link
                className="button button--secondary button--lg"
                to="/docs/intro">
                Get Started - 5min ⏱️
              </Link>
              <Link
                className="button button--outline button--lg"
                to="/docs/performance/benchmarks"
                style={{marginLeft: '1rem'}}>
                View Benchmarks 🚀
              </Link>
            </div>
          </div>
          <div className={styles.heroVisualization}>
            <div className={styles.performanceCard}>
              <h3>Latest Benchmarks</h3>
              <div className={styles.benchmarkGrid}>
                <div className={styles.benchmarkItem}>
                  <div className={styles.benchmarkValue}>75.4µs</div>
                  <div className={styles.benchmarkLabel}>Vector Addition</div>
                  <div className={styles.benchmarkImprovement}>1.6x faster</div>
                </div>
                <div className={styles.benchmarkItem}>
                  <div className={styles.benchmarkValue}>26.7µs</div>
                  <div className={styles.benchmarkLabel}>Sum Operations</div>
                  <div className={styles.benchmarkImprovement}>3.9x faster</div>
                </div>
                <div className={styles.benchmarkItem}>
                  <div className={styles.benchmarkValue}>20.5ns</div>
                  <div className={styles.benchmarkLabel}>Column Access</div>
                  <div className={styles.benchmarkImprovement}>Zero-copy</div>
                </div>
              </div>
            </div>
          </div>
        </div>
        <div className={styles.heroStats}>
          <div className={styles.stat}>
            <div className={styles.statNumber}>4x</div>
            <div className={styles.statLabel}>Faster Processing</div>
          </div>
          <div className={styles.stat}>
            <div className={styles.statNumber}>45%</div>
            <div className={styles.statLabel}>Less Memory</div>
          </div>
          <div className={styles.stat}>
            <div className={styles.statNumber}>128</div>
            <div className={styles.statLabel}>Tests Passing</div>
          </div>
          <div className={styles.stat}>
            <div className={styles.statNumber}>3</div>
            <div className={styles.statLabel}>Languages</div>
          </div>
        </div>
      </div>
    </header>
  );
}

export default function Home(): JSX.Element {
  const {siteConfig} = useDocusaurusContext();
  return (
    <Layout
      title={`Hello from ${siteConfig.title}`}
      description="Lightning-fast data processing library for Rust, Python & JavaScript">
      <HomepageHeader />
      <main>
        <HomepageFeatures />
        <PerformanceComparison />
      </main>
    </Layout>
  );
}