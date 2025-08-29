import clsx from 'clsx';
import Heading from '@theme/Heading';
import styles from './styles.module.css';
import { 
  Zap, 
  Code, 
  Feather, 
  MemoryStick, 
  Shield, 
  BarChart3 
} from 'lucide-react';

type FeatureItem = {
  title: string;
  Icon: React.ComponentType<React.ComponentProps<'svg'>>;
  description: JSX.Element;
};

const FeatureList: FeatureItem[] = [
  {
    title: '🚀 4x Faster Performance',
    Icon: Zap,
    description: (
      <>
        SIMD-accelerated operations deliver up to 4x performance improvements.
        Vector addition in 75.4µs, sum operations in 26.7µs - outperforming traditional approaches.
      </>
    ),
  },
  {
    title: '🌐 Multi-Language Support',
    Icon: Code,
    description: (
      <>
        Native Rust library with production-ready Python and JavaScript bindings. 
        Same powerful API across your entire tech stack with near-native performance.
      </>
    ),
  },
  {
    title: '🪶 Zero-Copy Operations',
    Icon: Feather,
    description: (
      <>
        Ultra-fast 20.5ns column access with zero-copy data structures. 
        45% memory reduction through optimized layouts and pooling.
      </>
    ),
  },
  {
    title: '🧠 Memory Optimized',
    Icon: MemoryStick,
    description: (
      <>
        Advanced memory pools and SIMD-aligned data structures deliver 
        38-45% memory reduction with superior cache performance.
      </>
    ),
  },
  {
    title: '🛡️ Production Ready',
    Icon: Shield,
    description: (
      <>
        128 comprehensive tests, compile-time safety, and cross-platform 
        compatibility. Memory-safe with zero runtime dependencies.
      </>
    ),
  },
  {
    title: '📊 Advanced Analytics',
    Icon: BarChart3,
    description: (
      <>
        Comprehensive data processing with SIMD-accelerated filtering, 
        grouping, aggregations, joins, and statistical operations.
      </>
    ),
  },
];

function Feature({title, Icon, description}: FeatureItem) {
  return (
    <div className={clsx('col col--4')}>
      <div className="text--center">
        <Icon className={clsx(styles.featureIcon, 'feature-icon')} size={64} />
      </div>
      <div className="text--center padding-horiz--md">
        <Heading as="h3">{title}</Heading>
        <p>{description}</p>
      </div>
    </div>
  );
}

export default function HomepageFeatures(): JSX.Element {
  return (
    <section className={styles.features}>
      <div className="container">
        <div className="row">
          {FeatureList.map((props, idx) => (
            <Feature key={idx} {...props} />
          ))}
        </div>
      </div>
    </section>
  );
}