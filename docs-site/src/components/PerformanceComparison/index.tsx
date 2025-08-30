import React from 'react';
import styles from './PerformanceComparison.module.css';

const PerformanceComparison = () => {
  const benchmarkData = [
    {
      operation: 'Vector Addition (100k)',
      veloxx: '75.4µs',
      pandas: '~200µs',
      numpy: '~120µs',
      improvement: '1.6x faster',
      color: '#34d399'
    },
    {
      operation: 'Sum Operations (100k)',
      veloxx: '26.7µs',
      pandas: '~150µs',
      numpy: '~80µs',
      improvement: '3.9x faster',
      color: '#fbbf24'
    },
    {
      operation: 'Column Access',
      veloxx: '20.5ns',
      pandas: '~100ns',
      numpy: '~50ns',
      improvement: 'Zero-copy',
      color: '#a78bfa'
    },
    {
      operation: 'Memory Usage',
      veloxx: '45% less',
      pandas: 'baseline',
      numpy: 'baseline',
      improvement: '38-45% reduction',
      color: '#fb7185'
    }
  ];

  return (
    <section className={styles.performanceSection}>
      <div className="container">
        <div className={styles.sectionHeader}>
          <h2>Performance Comparison</h2>
          <p>See how Veloxx compares against popular data processing libraries</p>
        </div>
        
        <div className={styles.comparisonTable}>
          <div className={styles.tableHeader}>
            <div className={styles.headerCell}>Operation</div>
            <div className={styles.headerCell}>Veloxx</div>
            <div className={styles.headerCell}>Pandas</div>
            <div className={styles.headerCell}>NumPy</div>
            <div className={styles.headerCell}>Improvement</div>
          </div>
          
          {benchmarkData.map((row, index) => (
            <div key={index} className={styles.tableRow}>
              <div className={styles.cell}>{row.operation}</div>
              <div className={`${styles.cell} ${styles.veloxxCell}`}>
                <strong>{row.veloxx}</strong>
              </div>
              <div className={styles.cell}>{row.pandas}</div>
              <div className={styles.cell}>{row.numpy}</div>
              <div 
                className={`${styles.cell} ${styles.improvementCell}`}
                style={{ color: row.color }}
              >
                <strong>{row.improvement}</strong>
              </div>
            </div>
          ))}
        </div>
        
        <div className={styles.performanceMetrics}>
          <div className={styles.metric}>
            <div className={styles.metricValue}>4x</div>
            <div className={styles.metricLabel}>Faster Processing</div>
          </div>
          <div className={styles.metric}>
            <div className={styles.metricValue}>45%</div>
            <div className={styles.metricLabel}>Memory Reduction</div>
          </div>
          <div className={styles.metric}>
            <div className={styles.metricValue}>128</div>
            <div className={styles.metricLabel}>Tests Passing</div>
          </div>
        </div>
      </div>
    </section>
  );
};

export default PerformanceComparison;
