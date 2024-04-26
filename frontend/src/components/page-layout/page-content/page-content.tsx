import { Flex, Layout, theme } from 'antd'
import Album, { AlbumProps } from '../../album/album';
import { useEffect, useState } from 'react';
import axios from 'axios';

const { Content } = Layout;

const PageContent = () => {
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
    )
};

export default PageContent;
