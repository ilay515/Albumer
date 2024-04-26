import { Layout } from 'antd'
import './styles.css'
import Title from 'antd/es/typography/Title';
import Sider from 'antd/es/layout/Sider';
import PageContent from './page-content/page-content';

const PageLayout = () => {
    return (
        <Layout className='page-layout' style={{ minHeight: '100vh' }}>
            <Sider>
                <Title>
                    Albumer
                </Title>
            </Sider>
            <PageContent />
        </Layout>
    )
};

export default PageLayout;

