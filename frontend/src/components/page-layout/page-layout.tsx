import { Flex, Layout, theme } from 'antd'
import './styles.css'
import Title from 'antd/es/typography/Title';
import Album from '../album/album';
import { useEffect, useState } from 'react';
import axios from 'axios';
import { AlbumProps } from '../album/album';
import Sider from 'antd/es/layout/Sider';


const { Content } = Layout;

const PageLayout = () => {
    const {
        token: { colorBgContainer, borderRadiusLG },
    } = theme.useToken();
    const [albums, setAlbums] = useState<AlbumProps[]>([]);
    const getData = async () => {
        const { data } = await axios.get(`http://127.0.0.1:8000/library`);
        console.log(data)
        setAlbums(data);
    };
    useEffect(() => {
        getData();
    }, []);

    return (
        <Layout className='page-layout' style={{ minHeight: '100vh' }}>
            <Sider>
                <Title>
                    Albumer
                </Title>
            </Sider>
            <Layout>
                <Content
                    style={{
                        margin: '24px 16px',
                        padding: 24,
                        minHeight: 280,
                        background: colorBgContainer,
                        borderRadius: borderRadiusLG,
                    }}
                >
                    <Flex wrap="wrap" gap="small">
                        {albums.map((album: AlbumProps) => <Album {...album} />)}
                    </Flex>
                </Content>
            </Layout>
        </Layout>
    )
};

export default PageLayout;

