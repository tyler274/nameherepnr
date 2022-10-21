use crate::kernel::id_string::IdString;
use std::marker::PhantomData;

pub struct ArchArgs<R> {
    phantom: PhantomData<R>,
}

impl<R> ArchArgs<R> {
    pub const fn new() -> Self {
        Self {
            phantom: PhantomData,
        }
    }
}

impl<R> const Default for ArchArgs<R> {
    fn default() -> Self {
        Self::new()
    }
}

// The specification of the Arch API (pure virtual)
pub trait ArchAPI<R> {
    // Basic config
    fn arch_id(&self) -> IdString {
        Default::default()
    }
    fn get_chip_name(&self) -> String {
        Default::default()
    }
    fn arch_args(&self) -> ArchArgs<R> {
        Default::default()
    }
    //    virtual IdString archArgsToId(typename R::ArchArgsT args) const = 0;
    //    virtual int getGridDimX() const = 0;
    //    virtual int getGridDimY() const = 0;
    //    virtual int getTileBelDimZ(int x, int y) const = 0;
    //    virtual int getTilePipDimZ(int x, int y) const = 0;
    //    virtual char getNameDelimiter() const = 0;
    // Bel methods
    //    virtual typename R::AllBelsRangeT getBels() const = 0;
    //    virtual IdStringList getBelName(BelId bel) const = 0;
    //    virtual BelId getBelByName(IdStringList name) const = 0;
    //    virtual uint32_t getBelChecksum(BelId bel) const = 0;
    //    virtual void bindBel(BelId bel, CellInfo *cell, PlaceStrength strength) = 0;
    //    virtual void unbindBel(BelId bel) = 0;
    //    virtual Loc getBelLocation(BelId bel) const = 0;
    //    virtual BelId getBelByLocation(Loc loc) const = 0;
    //    virtual typename R::TileBelsRangeT getBelsByTile(int x, int y) const = 0;
    //    virtual bool getBelGlobalBuf(BelId bel) const = 0;
    //    virtual bool checkBelAvail(BelId bel) const = 0;
    //    virtual CellInfo *getBoundBelCell(BelId bel) const = 0;
    //    virtual CellInfo *getConflictingBelCell(BelId bel) const = 0;
    //    virtual IdString getBelType(BelId bel) const = 0;
    //    virtual bool getBelHidden(BelId bel) const = 0;
    //    virtual typename R::BelAttrsRangeT getBelAttrs(BelId bel) const = 0;
    //    virtual WireId getBelPinWire(BelId bel, IdString pin) const = 0;
    //    virtual PortType getBelPinType(BelId bel, IdString pin) const = 0;
    //    virtual typename R::BelPinsRangeT getBelPins(BelId bel) const = 0;
    //    virtual typename R::CellBelPinRangeT getBelPinsForCellPin(const CellInfo *cell_info, IdString pin) const = 0;
    // Wire methods
    //    virtual typename R::AllWiresRangeT getWires() const = 0;
    //    virtual WireId getWireByName(IdStringList name) const = 0;
    //    virtual IdStringList getWireName(WireId wire) const = 0;
    //    virtual IdString getWireType(WireId wire) const = 0;
    //    virtual typename R::WireAttrsRangeT getWireAttrs(WireId) const = 0;
    //    virtual typename R::DownhillPipRangeT getPipsDownhill(WireId wire) const = 0;
    //    virtual typename R::UphillPipRangeT getPipsUphill(WireId wire) const = 0;
    //    virtual typename R::WireBelPinRangeT getWireBelPins(WireId wire) const = 0;
    //    virtual uint32_t getWireChecksum(WireId wire) const = 0;
    //    virtual void bindWire(WireId wire, NetInfo *net, PlaceStrength strength) = 0;
    //    virtual void unbindWire(WireId wire) = 0;
    //    virtual bool checkWireAvail(WireId wire) const = 0;
    //    virtual NetInfo *getBoundWireNet(WireId wire) const = 0;
    //    virtual WireId getConflictingWireWire(WireId wire) const = 0;
    //    virtual NetInfo *getConflictingWireNet(WireId wire) const = 0;
    //    virtual DelayQuad getWireDelay(WireId wire) const = 0;
    // Pip methods
    //    virtual typename R::AllPipsRangeT getPips() const = 0;
    //    virtual PipId getPipByName(IdStringList name) const = 0;
    //    virtual IdStringList getPipName(PipId pip) const = 0;
    //    virtual IdString getPipType(PipId pip) const = 0;
    //    virtual typename R::PipAttrsRangeT getPipAttrs(PipId) const = 0;
    //    virtual uint32_t getPipChecksum(PipId pip) const = 0;
    //    virtual void bindPip(PipId pip, NetInfo *net, PlaceStrength strength) = 0;
    //    virtual void unbindPip(PipId pip) = 0;
    //    virtual bool checkPipAvail(PipId pip) const = 0;
    //    virtual bool checkPipAvailForNet(PipId pip, NetInfo *net) const = 0;
    //    virtual NetInfo *getBoundPipNet(PipId pip) const = 0;
    //    virtual WireId getConflictingPipWire(PipId pip) const = 0;
    //    virtual NetInfo *getConflictingPipNet(PipId pip) const = 0;
    //    virtual WireId getPipSrcWire(PipId pip) const = 0;
    //    virtual WireId getPipDstWire(PipId pip) const = 0;
    //    virtual DelayQuad getPipDelay(PipId pip) const = 0;
    //    virtual Loc getPipLocation(PipId pip) const = 0;
    // Group methods
    //    virtual GroupId getGroupByName(IdStringList name) const = 0;
    //    virtual IdStringList getGroupName(GroupId group) const = 0;
    //    virtual typename R::AllGroupsRangeT getGroups() const = 0;
    //    virtual typename R::GroupBelsRangeT getGroupBels(GroupId group) const = 0;
    //    virtual typename R::GroupWiresRangeT getGroupWires(GroupId group) const = 0;
    //    virtual typename R::GroupPipsRangeT getGroupPips(GroupId group) const = 0;
    //    virtual typename R::GroupGroupsRangeT getGroupGroups(GroupId group) const = 0;
    // Delay Methods
    //    virtual delay_t predictDelay(BelId src_bel, IdString src_pin, BelId dst_bel, IdString dst_pin) const = 0;
    //    virtual delay_t getDelayEpsilon() const = 0;
    //    virtual delay_t getRipupDelayPenalty() const = 0;
    //    virtual float getDelayNS(delay_t v) const = 0;
    //    virtual delay_t getDelayFromNS(float ns) const = 0;
    //    virtual uint32_t getDelayChecksum(delay_t v) const = 0;
    //    virtual bool getBudgetOverride(const NetInfo *net_info, const PortRef &sink, delay_t &budget) const = 0;
    //    virtual delay_t estimateDelay(WireId src, WireId dst) const = 0;
    //    virtual ArcBounds getRouteBoundingBox(WireId src, WireId dst) const = 0;
    // Decal methods
    //    virtual typename R::DecalGfxRangeT getDecalGraphics(DecalId decal) const = 0;
    //    virtual DecalXY getBelDecal(BelId bel) const = 0;
    //    virtual DecalXY getWireDecal(WireId wire) const = 0;
    //    virtual DecalXY getPipDecal(PipId pip) const = 0;
    //    virtual DecalXY getGroupDecal(GroupId group) const = 0;
    // Cell timing methods
    //    virtual bool getCellDelay(const CellInfo *cell, IdString fromPort, IdString toPort, DelayQuad &delay) const = 0;
    //    virtual TimingPortClass getPortTimingClass(const CellInfo *cell, IdString port, int &clockInfoCount) const = 0;
    //    virtual TimingClockingInfo getPortClockingInfo(const CellInfo *cell, IdString port, int index) const = 0;
    // Placement validity checks
    //    virtual bool isValidBelForCellType(IdString cell_type, BelId bel) const = 0;
    //    virtual IdString getBelBucketName(BelBucketId bucket) const = 0;
    //    virtual BelBucketId getBelBucketByName(IdString name) const = 0;
    //    virtual BelBucketId getBelBucketForBel(BelId bel) const = 0;
    //    virtual BelBucketId getBelBucketForCellType(IdString cell_type) const = 0;
    //    virtual bool isBelLocationValid(BelId bel) const = 0;
    //    virtual typename R::CellTypeRangeT getCellTypes() const = 0;
    //    virtual typename R::BelBucketRangeT getBelBuckets() const = 0;
    //    virtual typename R::BucketBelRangeT getBelsInBucket(BelBucketId bucket) const = 0;
    // Cluster methods
    //    virtual CellInfo *getClusterRootCell(ClusterId cluster) const = 0;
    //    virtual ArcBounds getClusterBounds(ClusterId cluster) const = 0;
    //    virtual Loc getClusterOffset(const CellInfo *cell) const = 0;
    //    virtual bool isClusterStrict(const CellInfo *cell) const = 0;
    //    virtual bool getClusterPlacement(ClusterId cluster, BelId root_bel,
    //    std::vector<std::pair<CellInfo *, BelId>> &placement) const = 0;

    // Flow methods
    fn pack(&self) -> bool {
        false
    }
    fn place(&self) -> bool {
        false
    }
    fn route(&self) -> bool {
        false
    }
    fn assign_arch_info(&self) -> bool {
        false
    }
}
