import { Card, Descriptions, Flex, Image, Tag } from 'antd';

interface Artist {
    name: string
}

interface Image {
    url: string,
    height: number,
    width: number
}

export interface AlbumProps {
    name: String,
    artists: Artist[],
    images: Image[],
    release_date: String,
    popularity: number,
    genres: String[]
}

const Album = ({ name, artists, images, release_date, popularity, genres }: AlbumProps) => {
    return (
        <div>
            <Card hoverable style={{ width: 300 }} cover={
                <Image alt="Album Cover" src={images[0].url} />}
            >
                <Flex vertical>
                    <Descriptions title={name} column={1}>
                        <Descriptions.Item>{artists.map(artist => artist.name)}</Descriptions.Item>
                        <Descriptions.Item label="Release Date">{release_date}</Descriptions.Item>
                        <Descriptions.Item label="rating">{popularity / 10}</Descriptions.Item>
                        {genres.map(genre => 
                            <Descriptions.Item>
                                <Tag color="red">
                                    {genre}
                                </Tag>
                            </Descriptions.Item>
                        )}
                    </Descriptions>
                </Flex>
            </Card>
        </div>
    )
};

export default Album;

