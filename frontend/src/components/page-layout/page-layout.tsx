import { Col, Flex, Layout, Row } from 'antd'
import './styles.css'
import Title from 'antd/es/typography/Title';
import Album from '../album/album';
import { useEffect, useState } from 'react';
import axios from 'axios';
import { AlbumProps } from '../album/album';


const { Header, Content } = Layout;

const PageLayout = () => {
    const [albums, setAlbums] = useState<AlbumProps[]>([]);
    const getData = async () => {
        const { data } = await axios.get(`http://127.0.0.1:8000/albums`);
        console.log(data)
        setAlbums(data);
    };
    useEffect(() => {
        getData();
    }, []);

    return (
        <Layout className='page-layout'>
            <Header >
                <Flex className='title' justify={'center'} align={'center'}>
                    <Title >
                        Albumer
                    </Title>
                </Flex>
            </Header>
            <Content >
                <Flex wrap="wrap" gap="small">
                    {albums.map((album: AlbumProps) => <Album {...album} />)}
                </Flex>
            </Content>
        </Layout>
    )
};

export default PageLayout;

